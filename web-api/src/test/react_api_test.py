import json
import pytest
from src import app


@pytest.fixture
def client():
    return app.test_client()


# Assuming you have the rest of your imports and fixtures as before

# Test POST with valid coordinates
def test_post_coordinates_valid(client):
    # Sample valid data
    data = {
        "northEast": {"lat": "55.88898321305664", "lng": "-4.199352264404298"},
        "southWest": {"lat": "55.830215663223996", "lng": "-4.3157386779785165"}
    }
    
    response = client.post('/api/post-coordinates', data=json.dumps(data), content_type='application/json')
    assert response.status_code == 200
    assert response.json == {'success': 'true', 'message': 'Coordinates received'}

# Test POST with invalid data structure (missing 'northEast' or 'southWest')
def test_post_coordinates_invalid_structure(client):
    # Sample invalid data
    data = {"north": {"lat": "55.88898321305664", "lng": "-4.199352264404298"}}
    
    response = client.post('/api/post-coordinates', data=json.dumps(data), content_type='application/json')
    assert response.status_code != 200

# Test POST with invalid data values (incorrect types for lat/lng)
def test_post_coordinates_invalid_values(client):
    # Sample invalid data with string instead of float
    data = {
        "northEast": {"lat": "invalid_latitude", "lng": "-4.199352264404298"},
        "southWest": {"lat": "55.830215663223996", "lng": "invalid_longitude"}
    }
    
    response = client.post('/api/post-coordinates', data=json.dumps(data), content_type='application/json')
    assert response.status_code == 400  # Assuming your error handling sends back a 400 for bad input

# Test POST with empty data
def test_post_coordinates_empty_data(client):
    response = client.post('/api/post-coordinates', data={}, content_type='application/json')
    assert response.status_code == 400  # Again, assuming you handle empty data with a 400 status code

# Add more tests as necessary for other edge cases you want to handle
