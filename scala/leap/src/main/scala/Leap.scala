object Leap {
  def leapYear(year: Int): Boolean = {
    evenlyDivisible(year, 4) && (!evenlyDivisible(year, 100) | evenlyDivisible(year, 400))
  }
  
  private def evenlyDivisible(number: Int, divisor: Int): Boolean = number % divisor == 0
}
