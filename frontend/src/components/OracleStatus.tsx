import React, { useState, useEffect } from 'react';
import { OracleStatus, WeatherData, TraitCalculation } from '../types';

export const OracleStatus: React.FC = () => {
  const [status, setStatus] = useState<OracleStatus | null>(null);
  const [weather, setWeather] = useState<WeatherData | null>(null);
  const [traits, setTraits] = useState<TraitCalculation | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchAllData();
    const interval = setInterval(fetchAllData, 30000); // Update every 30 seconds
    return () => clearInterval(interval);
  }, []);

  const fetchAllData = async () => {
    try {
      const [statusRes, weatherRes, traitsRes] = await Promise.all([
        fetch('/api/oracle/status'),
        fetch('/api/oracle/weather'),
        fetch('/api/oracle/traits')
      ]);

      const statusData = await statusRes.json();
      const weatherData = await weatherRes.json();
      const traitsData = await traitsRes.json();

      if (statusData.success) setStatus(statusData.data);
      if (weatherData.success) setWeather(weatherData.data);
      if (traitsData.success) setTraits(traitsData.data);
    } catch (error) {
      console.error('Failed to fetch oracle data:', error);
    } finally {
      setLoading(false);
    }
  };

  const triggerUpdate = async () => {
    try {
      const response = await fetch('/api/oracle/update', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          nft_mint_addresses: [],
          force_update: true
        })
      });

      const result = await response.json();
      if (result.success) {
        alert('Oracle update triggered successfully!');
        fetchAllData();
      } else {
        alert(`Update failed: ${result.error}`);
      }
    } catch (error) {
      console.error('Failed to trigger update:', error);
      alert('Failed to trigger update');
    }
  };

  if (loading) {
    return (
      <div className="flex justify-center items-center h-64">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-500"></div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-4xl font-bold text-white">Oracle Status</h1>
        <button 
          onClick={triggerUpdate}
          className="solana-button"
        >
          Trigger Update
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Oracle Status Card */}
        <div className="nft-card">
          <h3 className="text-lg font-semibold text-white mb-4">Oracle Service</h3>
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-gray-400">Status:</span>
              <span className={`px-2 py-1 rounded text-xs ${
                status?.is_running ? 'bg-green-500 text-white' : 'bg-red-500 text-white'
              }`}>
                {status?.is_running ? 'Running' : 'Stopped'}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-400">Total Updates:</span>
              <span className="text-white">{status?.total_updates || 0}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-400">Error Count:</span>
              <span className="text-white">{status?.error_count || 0}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-400">Update Interval:</span>
              <span className="text-white">{status?.update_interval}s</span>
            </div>
            {status?.last_update && (
              <div className="flex justify-between">
                <span className="text-gray-400">Last Update:</span>
                <span className="text-white text-sm">
                  {new Date(status.last_update).toLocaleString()}
                </span>
              </div>
            )}
            {status?.next_update && (
              <div className="flex justify-between">
                <span className="text-gray-400">Next Update:</span>
                <span className="text-white text-sm">
                  {new Date(status.next_update).toLocaleString()}
                </span>
              </div>
            )}
          </div>
        </div>

        {/* Weather Data Card */}
        <div className="nft-card">
          <h3 className="text-lg font-semibold text-white mb-4">Current Weather</h3>
          {weather ? (
            <div className="space-y-3">
              <div className="flex justify-between">
                <span className="text-gray-400">Location:</span>
                <span className="text-white">{weather.location}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">Temperature:</span>
                <span className="text-white">{weather.temperature}°C</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">Humidity:</span>
                <span className="text-white">{weather.humidity}%</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">Wind Speed:</span>
                <span className="text-white">{weather.wind_speed} m/s</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">Condition:</span>
                <span className="text-white capitalize">{weather.weather_condition}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">Updated:</span>
                <span className="text-white text-sm">
                  {new Date(weather.timestamp).toLocaleTimeString()}
                </span>
              </div>
            </div>
          ) : (
            <div className="text-gray-400">No weather data available</div>
          )}
        </div>

        {/* Calculated Traits Card */}
        <div className="nft-card">
          <h3 className="text-lg font-semibold text-white mb-4">Calculated Traits</h3>
          {traits ? (
            <div className="space-y-3">
              <div className="flex justify-between">
                <span className="text-gray-400">Background:</span>
                <span className="text-white">{traits.calculated_traits.background}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">Mood:</span>
                <span className="text-white">{traits.calculated_traits.mood}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">Activity:</span>
                <span className="text-white">{traits.calculated_traits.activity}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">Weather Effect:</span>
                <span className="text-white">{traits.calculated_traits.weather_effect}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">Time of Day:</span>
                <span className="text-white">{traits.calculated_traits.time_of_day}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">Power Level:</span>
                <span className="text-white">{traits.calculated_traits.power_level}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">Rarity Score:</span>
                <span className="text-white">{traits.calculated_traits.rarity_score}</span>
              </div>
            </div>
          ) : (
            <div className="text-gray-400">No trait data available</div>
          )}
        </div>
      </div>

      {status?.status_message && (
        <div className="nft-card">
          <h3 className="text-lg font-semibold text-white mb-2">Status Message</h3>
          <p className="text-gray-300">{status.status_message}</p>
        </div>
      )}
    </div>
  );
};
