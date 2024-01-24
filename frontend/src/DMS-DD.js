const Decimal = require('decimal.js');

function dmsToDecimal(degrees, minutes, seconds, direction) {
  const decimalDegrees = new Decimal(degrees)
    .plus(new Decimal(minutes).dividedBy(60))
    .plus(new Decimal(seconds).dividedBy(3600));

  // Adjust for the direction (N, S, E, W)
  if (direction === 'S' || direction === 'W') {
    return decimalDegrees.negated();
  }

  return decimalDegrees;
}
