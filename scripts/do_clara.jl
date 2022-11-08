#!/usr/bin/env julia --startup-file=no
import Pkg
Pkg.activate(".")

using Dates
using OAuth
using JSON

# args order is [number of messages], [letter number]

creds = Dict(
  :consumer_key => "qAxTANq8dqpN9p8qh76UWllCw",
  :consumer_secret => "RONNPwufK4S4MPPd3CohL6kdQ3pjIctGL4yvt4OP5NmAmF2dP7",
  :oauth_token => "1223167297495285760-GsTH08cn7gN2rm2Ks0DIWDRaLL8HHZ",
  :oauth_secret => "sF1VMAT3rVp5yp5mr7d23HSbaAyd6vPUyWPHDegjnT5Kg"
)

get_oauth(endpoint::String, options::Dict) = oauth_request_resource(
  endpoint, "GET", options, 
  creds[:consumer_key], creds[:consumer_secret], creds[:oauth_token], creds[:oauth_secret])



dms = get_oauth("https://api.twitter.com/1.1/direct_messages/events/list.json", 
  Dict("count" => "20")).body |> String |> JSON.parse |> 
  res -> res["events"] |>
  events ->  [dm["message_create"]["message_data"] for dm in events if dm["message_create"]["sender_id"] == "1218322617117106176"]

counter = 0
for dm in dms[1:parse(Int, ARGS[1])]
  global counter
  # extract images
  media_url = dm["attachment"]["media"]["media_url_https"]
  image = get_oauth(media_url, Dict()).body
  open("public/clara/img/$(Date(now()))_Clara_0_$counter.jpg", "w") do f
    counter += 1
    write(f, image)
  end
  
  # extract text
  if match(r"^ https://t.co/\w+$", dm["text"]) === nothing
    text = replace(dm["text"], r"\bhttps://t.co/\w+\b" => "")
    open("letters/$(ARGS[2])_$(Date(now()))_Clara", "w") do f
      write(f, text)
    end
  end
end
