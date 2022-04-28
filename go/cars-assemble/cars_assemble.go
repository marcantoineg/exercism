package cars

const (
	singleCarCost        = 10_000
	groupRebateCarCount  = 10
	groupRebateTotalCost = 95_000
)

// CalculateWorkingCarsPerHour calculates how many working cars are
// produced by the assembly line every hour
func CalculateWorkingCarsPerHour(productionRate int, successRate float64) float64 {
	return float64(productionRate) * (successRate / 100.0)
}

// CalculateWorkingCarsPerMinute calculates how many working cars are
// produced by the assembly line every minute
func CalculateWorkingCarsPerMinute(productionRate int, successRate float64) int {
	return int(CalculateWorkingCarsPerHour(productionRate, successRate)) / 60
}

// CalculateCost works out the cost (of producing the given number of cars
func CalculateCost(carsCount int) uint {
	if carsCount <= 0 {
		return 0
	}
	return uint((carsCount / groupRebateCarCount * groupRebateTotalCost) + (carsCount % groupRebateCarCount * singleCarCost))
}
