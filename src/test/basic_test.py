import pytest
from src import app


def test_basic():
    resp = app.test_client().get("/")
    assert b"<form" in resp.data
