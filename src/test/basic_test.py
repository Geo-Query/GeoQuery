import pytest
from src import app


@pytest.fixture
def client():
    return app.test_client()

def test_basic():
    resp = app.test_client().get("/")
    assert b"<form" in resp.data
