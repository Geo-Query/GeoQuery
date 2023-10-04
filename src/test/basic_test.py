import pytest
from src import app


def test_basic():
    resp = app.test_client().get("/")
    assert b"Foo Index Page" in resp.data
