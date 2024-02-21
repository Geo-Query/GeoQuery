import Decimal from 'decimal.js';

function dmdmToDecimal(degrees: number, minutes: number, direction: string): string {
  const decimalDegrees: Decimal = new Decimal(degrees)
    .plus(new Decimal(minutes).dividedBy(60));

  // Adjust for the direction (N, S, E, W)
  if (direction === 'S' || direction === 'W') {
    return decimalDegrees.negated().toString();
  }

  return decimalDegrees.toString();
}

export default dmdmToDecimal;