// Package weather provides information about forecasts.
package weather

// CurrentCondition represents the current weather condition for weather forecasts.
var CurrentCondition string
// CurrentLocation represents the current location for weather forecasts.
var CurrentLocation string

// Forecast returns a shot description of the current forecast for a given city and condition.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
