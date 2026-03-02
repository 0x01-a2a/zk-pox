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

interface SpoofAnalysis {
  total_points: number;
  teleport_count: number;
  zero_noise_count: number;
  impossible_velocity_count: number;
  suspicion_score: number;
  verdict: 'Clean' | 'Suspicious' | 'LikelySpoofed';
}

interface ZkPoxModuleInterface {
  getGpsStats(): Promise<GpsStats>;
  generateProof(requestJson: string): Promise<string>;
  verifyProof(resultJson: string): Promise<boolean>;
  analyzeSpoofRisk(days: number): Promise<string>;
  countNightsNear(
    centerLat: number,
    centerLng: number,
    radiusM: number,
    days: number,
  ): Promise<number>;
}

const { ZkPoxModule } = NativeModules;

export default ZkPoxModule as ZkPoxModuleInterface;
export type {
  GpsStats,
  ProofRequest,
  SpoofAnalysis,
  ZkPoxModuleInterface,
};
