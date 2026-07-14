require 'sinatra'
require 'json'

set :port, ENV['PORT'] || 3000
set :bind, '0.0.0.0'

get '/' do
  content_type :json
  { message: 'Hello from Vessl Ruby Sinatra Example!' }.to_json
end
