package server

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHealthzReturnsOK(t *testing.T) {
	router := NewRouter()
	recorder := httptest.NewRecorder()
	request := httptest.NewRequest(http.MethodGet, "/healthz", nil)

	router.ServeHTTP(recorder, request)

	if recorder.Code != http.StatusOK {
		t.Fatalf("expected status %d, got %d", http.StatusOK, recorder.Code)
	}

	var body map[string]string
	if err := json.Unmarshal(recorder.Body.Bytes(), &body); err != nil {
		t.Fatalf("expected JSON body: %v", err)
	}

	if body["status"] != "ok" {
		t.Fatalf("unexpected status: %q", body["status"])
	}
}

func TestAPIHealthReturnsServiceStatus(t *testing.T) {
	router := NewRouter()
	recorder := httptest.NewRecorder()
	request := httptest.NewRequest(http.MethodGet, "/api/health", nil)

	router.ServeHTTP(recorder, request)

	if recorder.Code != http.StatusOK {
		t.Fatalf("expected status %d, got %d", http.StatusOK, recorder.Code)
	}

	var body map[string]string
	if err := json.Unmarshal(recorder.Body.Bytes(), &body); err != nil {
		t.Fatalf("expected JSON body: %v", err)
	}

	if body["service"] != "api" {
		t.Fatalf("unexpected service: %q", body["service"])
	}

	if body["status"] != "ok" {
		t.Fatalf("unexpected status: %q", body["status"])
	}
}
