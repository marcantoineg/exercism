
object Bearing.North
object Bearing.East
object Bearing.South
object Bearing.West

case class Robot(var direction: Bearing, var position: Set[Int])