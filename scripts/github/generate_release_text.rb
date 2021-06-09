# frozen_string_literal: true

require 'base64'
require 'changelogerator'
require 'erb'
require 'git'
require 'json'
require 'octokit'
require 'toml'
require_relative './lib.rb'

current_ref = ENV['GITHUB_REF']
token = ENV['GITHUB_TOKEN']
github_client = Octokit::Client.new(
  access_token: token
)

tetcoin_path = ENV['GITHUB_WORKSPACE'] + '/tetcoin/'

# Generate an ERB renderer based on the template .erb file
renderer = ERB.new(
  File.read(ENV['GITHUB_WORKSPACE'] + '/tetcoin/scripts/github/tetcoin_release.erb'),
  trim_mode: '<>'
)

# get ref of last tetcoin release
last_ref = 'refs/tags/' + github_client.latest_release(ENV['GITHUB_REPOSITORY']).tag_name

tetcoin_cl = Changelog.new(
  'tetcoin/tetcoin', last_ref, current_ref, token: token
)

# Gets the tetcore commit hash used for a given tetcoin ref
def get_tetcore_commit(client, ref)
  cargo = TOML::Parser.new(
    Base64.decode64(
      client.contents(
        ENV['GITHUB_REPOSITORY'],
        path: 'Cargo.lock',
        query: { ref: ref.to_s }
      ).content
    )
  ).parsed
  cargo['package'].find { |p| p['name'] == 'tc-cli' }['source'].split('#').last
end

tetcore_prev_sha = get_tetcore_commit(github_client, last_ref)
tetcore_cur_sha = get_tetcore_commit(github_client, current_ref)

tetcore_cl = Changelog.new(
  'tetcoin/tetcore', tetcore_prev_sha, tetcore_cur_sha,
  token: token,
  prefix: true
)

# Combine all changes into a single array and filter out companions
all_changes = (tetcoin_cl.changes + tetcore_cl.changes).reject do |c|
  c[:title] =~ /[Cc]ompanion/
end

# Set all the variables needed for a release

misc_changes = Changelog.changes_with_label(all_changes, 'B1-releasenotes')
client_changes = Changelog.changes_with_label(all_changes, 'B5-clientnoteworthy')
runtime_changes = Changelog.changes_with_label(all_changes, 'B7-runtimenoteworthy')

# Add the audit status for runtime changes
runtime_changes.each do |c|
  if c.labels.any? { |l| l[:name] == 'D1-audited👍' }
    c[:pretty_title] = "✅ `audited` #{c[:pretty_title]}"
    next
  end
  if c.labels.any? { |l| l[:name] == 'D9-needsaudit👮' }
    c[:pretty_title] = "❌ `AWAITING AUDIT` #{c[:pretty_title]}"
    next
  end
  if c.labels.any? { |l| l[:name] == 'D5-nicetohaveaudit⚠️' }
    c[:pretty_title] = "⏳ `pending non-critical audit` #{c[:pretty_title]}"
    next
  end
  c[:pretty_title] = "✅ `trivial` #{c[:pretty_title]}"
end

# The priority of users upgraded is determined by the highest-priority
# *Client* change
release_priority = Changelog.highest_priority_for_changes(client_changes)

# Pulled from the previous Github step
rustc_stable = ENV['RUSTC_STABLE']
rustc_nightly = ENV['RUSTC_NIGHTLY']
tetcoin_runtime = get_runtime('tetcoin', tetcoin_path)
metrocoin_runtime = get_runtime('metrocoin', tetcoin_path)
westend_runtime = get_runtime('westend', tetcoin_path)

# These json files should have been downloaded as part of the build-runtimes
# github action

tetcoin_json = JSON.parse(
  File.read(
    ENV['GITHUB_WORKSPACE'] + '/tetcoin-srtool-json/srtool_output.json'
  )
)

metrocoin_json = JSON.parse(
  File.read(
    ENV['GITHUB_WORKSPACE'] + '/metrocoin-srtool-json/srtool_output.json'
  )
)

puts renderer.result
