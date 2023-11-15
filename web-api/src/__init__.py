from flask import Flask,render_template,request, abort
from urllib import parse
import messageClasses
import struct
import zmq

app = Flask(__name__)

@app.route("/")
def home():
    return render_template("form.html")

@app.route("/search", methods=['GET'])
def search():
    """
    Handles the search endpoint by receiving URL encoded latitude and longitude values for 
    top_right and bottom_left corners.

    Query Parameters:
        - top_right: A string containing the latitude and longitude values separated by a comma for the top right corner. 
        - bottom_left: A string containing the latitude and longitude values separated by a comma for the bottom left corner. 

    Returns:
        - A string "Validated successfully" if the provided values are valid.
        - A 500 error response if the provided values are invalid or if any required parameter is missing.

    Exceptions:
        - ValueError: If the decoded latitude or longitude values are out of their valid ranges or if they cannot be parsed as floats.
    """
    
    encoded_top_right = request.args.get("top_right")
    encoded_bottom_left = request.args.get("bottom_left")
    if encoded_top_right is None or encoded_bottom_left is None:
        abort(500)
    
     # Decode the URL encoded values
    decoded_top_right = parse.unquote(encoded_top_right)
    decoded_bottom_left = parse.unquote(encoded_bottom_left)

    # Split and validate the latitudes and longitudes
    try:
        lat_right, long_right = map(float, decoded_top_right.split(','))
        lat_left, long_left = map(float, decoded_bottom_left.split(','))
        
        if not (-90 <= lat_right <= 90) or not (-90 <= lat_left <= 90):
            raise ValueError("Invalid latitude value")
        
        if not (-180 <= long_right <= 180) or not (-180 <= long_left <= 180):
            raise ValueError("Invalid longitude value")

    except ValueError as e:
        print(f"Error: {e}")
        abort(500)

    
    return "Validated successfully"


#This function can be called in order to send a specific amount of requests to the data parser, with a specific message

def data_parser_io(request_amount, message_to_send):
    context = zmq.Context()

    # Socket to talk to server
    socket = context.socket(zmq.REQ)
    socket.connect("tcp://localhost:5555")

    # Sends request to server
    for r in range(request_amount):
        print("Sending Request {r} ...")
        socket.send(message_to_send)

        # Checks if a return message has been sent, if it has, returns it
        message_received = socket.recv()

        if message_received is not None:
            print("Message Received")
            return message_received


# This function builds a lat and long message and returns it
# will have to change the struct code in order to implement BYTE_ORDER
def lat_long_builder(lat1, long1, lat2, long2):

    try:
        assert isinstance(lat1, float)
        assert isinstance(long1, float)
        assert isinstance(lat2, float)
        assert isinstance(long2, float)
    except AssertionError:
        raise AssertionError("Lat. and Long. Arguments should be float")

    float_array = [lat1, long1, lat2, long2]
    float_array_bytes = struct.pack('f' * len(float_array), *float_array)

    content = messageClasses.MessageContent(float_array_bytes)
    message = messageClasses.Message(content, 0)

    return message
