object SpaceAge {
    val earthYearInSeconds = 31557600
    val periodMultiplicators: Map[String, Double] = Map(
        "earth" -> 1,
        "mercury" -> 0.2408467,
        "venus" -> 0.61519726,
        "mars" -> 1.8808158,
        "jupiter" -> 11.862615,
        "saturn" -> 29.447498,
        "uranus" -> 84.016846,
        "neptune" -> 164.79132
    )

    def onEarth(secondsOld: Double) = secondsOld / earthYearInSeconds
    def onMercury(secondsOld: Double) = secondsOld / getYearInSeconds("mercury")
    def onVenus(secondsOld: Double) = secondsOld / getYearInSeconds("venus")
    def onMars(secondsOld: Double) = secondsOld / getYearInSeconds("mars")
    def onJupiter(secondsOld: Double) = secondsOld / getYearInSeconds("jupiter")
    def onSaturn(secondsOld: Double) = secondsOld / getYearInSeconds("saturn")
    def onUranus(secondsOld: Double) = secondsOld / getYearInSeconds("uranus")
    def onNeptune(secondsOld: Double) = secondsOld / getYearInSeconds("neptune")

    private def getYearInSeconds(planetName: String): Double = earthYearInSeconds * (periodMultiplicators(planetName))
}