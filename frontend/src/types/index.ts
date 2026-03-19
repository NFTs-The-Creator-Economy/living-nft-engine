export interface NFT {
  id: string;
  mint_address: string;
  owner_address: string;
  name: string;
  symbol: string;
  uri: string;
  traits: NFTTraits;
  created_at: string;
  updated_at: string;
  is_active: boolean;
}

export interface NFTTraits {
  background: number;
  mood: number;
  activity: number;
  weather_effect: number;
  time_of_day: number;
  special_event: number;
  power_level: number;
  rarity_score: number;
}

export interface TraitCalculation {
  weather_data: WeatherData;
  calculated_traits: NFTTraits;
  calculation_timestamp: string;
}

export interface OracleStatus {
  is_running: boolean;
  last_update: string | null;
  update_interval: number;
  next_update: string | null;
  total_updates: number;
  error_count: number;
  status_message: string;
}

export interface WeatherData {
  temperature: number;
  humidity: number;
  wind_speed: number;
  weather_condition: string;
  timestamp: string;
  location: string;
}

export interface MintNFTRequest {
  name: string;
  symbol: string;
  uri: string;
  initial_traits?: NFTTraits;
  owner_address: string;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  timestamp: string;
}

export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}
