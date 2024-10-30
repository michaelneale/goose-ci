#!/usr/bin/env ruby
require 'net/http'
require 'json'

API_KEY = 'your_api_key_here' # You need to sign up for a weather API service
BASE_URL = "http://api.weatherapi.com/v1/current.json?key=

# Default location is set to Australia, you can change it accordingly
LOCATION = 'Australia'

url = URI("#{BASE_URL}#{API_KEY}&q=#{LOCATION}")
response = Net::HTTP.get(url)
parsed = JSON.parse(response)

puts "Weather in #{LOCATION}:"
puts "Temperature: 	#{parsed['current']['temp_c']} °C"
puts "Condition: 	#{parsed['current']['condition']['text']}"
puts "Humidity: 	#{parsed['current']['humidity']}%"
puts "Wind: 	#{parsed['current']['wind_kph']} kph"
