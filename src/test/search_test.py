import pytest
from src import app


@pytest.fixture
def client():
    return app.test_client()



# 1. Test valid url with url encoded periods
def test_search_encoded(client):
    resp = client.get("/search?top_left=45%2E0%2C-100%2E0&bottom_right=35%2E0%2C-90%2E0")
    assert resp.status_code == 200
    assert b"Validated successfully" in resp.data

# 2. Test valid url with periods '.' instead of encoding
def test_search_periods(client):
    resp = client.get("/search?top_left=45.0,-100.0&bottom_right=35.0,-90.0")
    assert resp.status_code == 200
    assert b"Validated successfully" in resp.data

# 3. Test for missing `top_left` parameter
def test_search_missing_top_left(client):
    resp = client.get("/search?bottom_right=35.0,-90.0")
    assert resp.status_code == 500

# 4. Test for missing `bottom_right` parameter
def test_search_missing_bottom_right(client):
    resp = client.get("/search?top_left=45.0,-100.0")
    assert resp.status_code == 500

# 5. Test for invalid latitude
def test_search_invalid_latitude(client):
    # Using a latitude value of 100, which is out of range
    resp = client.get("/search?top_left=100.0,-100.0&bottom_right=35.0,-90.0")
    assert resp.status_code == 500

# 6. Test for invalid longitude
def test_search_invalid_longitude(client):
    # Using a longitude value of 200, which is out of range
    resp = client.get("/search?top_left=45.0,200.0&bottom_right=35.0,-90.0")
    assert resp.status_code == 500