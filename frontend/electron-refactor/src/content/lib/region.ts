export interface Coordinate {
    lat: number,
    long: number,
}

export interface Region {
    northWest: Coordinate,
    southEast: Coordinate,
}

export default class SelectedRegion {
    region?: Region;

    constructor(region?: Region) {
        this.region = region;
    }
}

export function validateAndConformCoordinate(initial: string): number {
    return parseFloat(initial);
}