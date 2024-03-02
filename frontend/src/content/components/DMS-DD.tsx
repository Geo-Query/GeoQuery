import Decimal from 'decimal.js';

function dmsToDecimal(degrees: number, minutes: number, seconds: number, direction: string): number {
  const decimalDegrees: Decimal = new Decimal(degrees)
    .plus(new Decimal(minutes).dividedBy(60))
    .plus(new Decimal(seconds).dividedBy(3600));

  // Adjust for the direction (N, S, E, W)
  if (direction === 'S' || direction === 'W') {
    return decimalDegrees.negated().toNumber();
  }

  return decimalDegrees.toNumber();
}

export default dmsToDecimal;
