require 'bunny'

connection = Bunny.new(automatically_recover: false)
connection.start

channel = connection.create_channel
queue = channel.queue('hello')

2000.times do
  channel.default_exchange.publish('Hello World!', routing_key: queue.name)
end

puts " [x] Sent 'Hello World!'"

connection.close
