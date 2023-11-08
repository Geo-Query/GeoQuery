from flask import Flask, render_template, request, abort, jsonify
from flask_cors import CORS
from urllib import parse

app = Flask(__name__)
CORS(app)

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

@app.route('/api/post-coordinates', methods=['POST'])
def get_coordinates():
    try:
        data = request.json

        if 'topRight' not in data or 'bottomLeft' not in data:
            raise ValueError("Invalid structure: 'topRight' and 'bottomRight' required")

        top_right = data['topRight']
        bottom_left = data['bottomLeft']

        if not all(key in top_right for key in ['lat', 'lng']) or not all(key in bottom_left for key in ['lat', 'lng']):
            raise ValueError("Invalid structure: 'lat' and 'lng' required for 'topRight' and 'bottomLeft'")

        ne_lat, ne_lng = float(top_right['lat']), float(top_right['lng'])
        sw_lat, sw_lng = float(bottom_left['lat']), float(bottom_left['lng'])

        return jsonify({'success': 'true', 'message': 'Coordinates received'})
    except ValueError as e:
        return jsonify({'success': 'false', 'message': str(e)}), 400