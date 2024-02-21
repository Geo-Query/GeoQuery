import Decimal from 'decimal.js';

function dmsToDecimal(degrees: number, minutes: number, seconds: number, direction: string): string {
  const decimalDegrees: Decimal = new Decimal(degrees)
    .plus(new Decimal(minutes).dividedBy(60))
    .plus(new Decimal(seconds).dividedBy(3600));

  // Adjust for the direction (N, S, E, W)
  if (direction === 'S' || direction === 'W') {
    return decimalDegrees.negated().toString();
  }

  return decimalDegrees.toString();
}

export default dmsToDecimal;
