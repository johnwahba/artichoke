#!/usr/bin/env ruby
# frozen_string_literal: true

begin
  Time
rescue NameError
  require 'time'
end

ITERATIONS = 50

def data
  File.read(File.join(__dir__, '..', 'fixtures', 'learnxinyminutes.txt'))
rescue StandardError
  $data # rubocop:disable Style/GlobalVars
end

# Timer that can log multiple iterations
class Stopwatch
  def initialize(name)
    @name = name
    @elapsed = 0
    @laps = 0
  end

  def lap
    start = Time.now
    yield
  ensure
    @elapsed += Time.now - start
    @laps += 1
  end

  def report
    ms = (@elapsed * 1e5).to_i / 1e2
    avg = (@elapsed / @laps * 1e5).to_i / 1e2
    "#{@name}: #{ms}ms elapsed in #{@laps} iterations (avg. #{avg}ms / iteration)"
  end
end

def bench(name, pattern)
  bench = data
  puts "\n#{name}: #{bench.scan(Regexp.compile(pattern)).size} matches"
  compile = Stopwatch.new('compile')
  scan = Stopwatch.new('scan')
  scan_with_block = Stopwatch.new('scan with block')
  ITERATIONS.times do
    print '.'
    regexp = compile.lap { Regexp.compile(pattern) }
    scan_count = scan.lap { bench.scan(regexp) }.size
    scan_with_block_count = scan_with_block.lap do
      count = 0
      bench.scan(regexp) { count += 1 }
      count
    end
    raise 'count mismatch' unless scan_count == scan_with_block_count
  end
  puts '', ''
  puts "    #{compile.report}"
  puts "    #{scan.report}"
  puts "    #{scan_with_block.report}"
end

puts "String#scan bench for #{RUBY_DESCRIPTION}"

bench('All', '.')

bench('Email', '[\w\.+-]+@[\w\.-]+\.[\w\.-]+')

begin
  # regex crate has pathological behavior when mixing unicode and \b
  bench('URI', 'https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}(?-u:\b)([-a-zA-Z0-9()@:%_\+.~#?&//=]*)')
rescue SyntaxError, RegexpError
  bench('URI', 'https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)')
end

begin
  # regex crate has pathological behavior when mixing unicode and \b
  bench('IP', '(?-u:\b)(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])(?-u:\b)')
rescue SyntaxError, RegexpError
  bench('IP', '\b(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\b')
end
