package dispatch

import "testing"

func TestGetFunction(t *testing.T) {
	for day := 1; day <= 12; day += 1 {
		solution := GetFunction(day)
		if solution == nil {
			t.Errorf("Got nil function pointer for day %d", day)
		}
	}
}
