import { NativeModules } from 'react-native';

interface GpsStats {
  totalPoints: number;
  oldestTimestamp?: number;
  newestTimestamp?: number;
  daysTracked: number;
}

interface ProofRequest {
  claim_type: number;
  center_lat: number;
  center_lng: number;
  radius_m: number;
  time_start: number;
  time_end: number;
  min_count: number;
  night_only?: boolean;
}

interface ZkPoxModuleInterface {
  getGpsStats(): Promise<GpsStats>;
  generateProof(requestJson: string): Promise<string>;
  countNightsNear(
    centerLat: number,
    centerLng: number,
    radiusM: number,
    days: number,
  ): Promise<number>;
}

const { ZkPoxModule } = NativeModules;

export default ZkPoxModule as ZkPoxModuleInterface;
export type { GpsStats, ProofRequest, ZkPoxModuleInterface };
