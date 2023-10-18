from flask import Flask,render_template,request, abort
from urllib import parse

app = Flask(__name__)

@app.route("/")
def home():
    return render_template("form.html")

@app.route("/search", methods=['GET'])
def search():
    """
    Handles the search endpoint by receiving URL encoded latitude and longitude values for 
    top_left and bottom_right corners.

    Query Parameters:
        - top_left: A string containing the latitude and longitude values separated by a comma for the top left corner. 
        - bottom_right: A string containing the latitude and longitude values separated by a comma for the bottom right corner. 

    Returns:
        - A string "Validated successfully" if the provided values are valid.
        - A 500 error response if the provided values are invalid or if any required parameter is missing.

    Exceptions:
        - ValueError: If the decoded latitude or longitude values are out of their valid ranges or if they cannot be parsed as floats.
    """
    
    encoded_top_left = request.args.get("top_left")
    encoded_bottom_right = request.args.get("bottom_right")
    if encoded_top_left is None or encoded_bottom_right is None:
        abort(500)
    
     # Decode the URL encoded values
    decoded_top_left = parse.unquote(encoded_top_left)
    decoded_bottom_right = parse.unquote(encoded_bottom_right)

    # Split and validate the latitudes and longitudes
    try:
        lat_left, long_left = map(float, decoded_top_left.split(','))
        lat_right, long_right = map(float, decoded_bottom_right.split(','))
        
        if not (-90 <= lat_left <= 90) or not (-90 <= lat_right <= 90):
            raise ValueError("Invalid latitude value")
        
        if not (-180 <= long_left <= 180) or not (-180 <= long_right <= 180):
            raise ValueError("Invalid longitude value")

    except ValueError as e:
        print(f"Error: {e}")
        abort(500)

    
    return "Validated successfully"

