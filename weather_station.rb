require 'net/http'
require 'json'

class WeatherStation
  BASE_URL = 'http://api.weatherapi.com/v1/forecast.json'.freeze
  API_KEY = 'your_api_key_here'
  LOCATION = 'New South Wales, Australia'

  def fetch_weather_forecast
    uri = URI("#{BASE_URL}?key=#{API_KEY}&q=#{LOCATION}&days=7")
    response = Net::HTTP.get(uri)
    forecast = JSON.parse(response)
    print_forecast(forecast)
  end

  private

  def print_forecast(forecast)
    forecast['forecast']['forecastday'].each do |day|
      date = day['date']
      condition = day['day']['condition']['text']
      max_temp = day['day']['maxtemp_c']
      min_temp = day['day']['mintemp_c']
      puts "#{date}: #{condition}, High: #{max_temp}°C, Low: #{min_temp}°C"
    end
  end
end

station = WeatherStation.new
station.fetch_weather_forecast
