const Decimal = require('decimal.js');

function dmdmToDecimal(degrees, minutes, decimalMinutes, direction) {
    const decimalDegrees = new Decimal(degrees)
    .plus(new Decimal(minutes).dividedBy(60))
    .plus(new Decimal(decimalMinutes).dividedBy(60));

    // Adjust for the direction (N, S, E, W)
    if (direction === 'S' || direction === 'W') {
    return decimalDegrees.negated();
  }

  return decimalDegrees;
}
