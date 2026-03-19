import React, { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import { NFT, NFTTraits } from '../types';
import { TraitBar } from './TraitBar';

export const TraitVisualizer: React.FC = () => {
  const { mintAddress } = useParams<{ mintAddress: string }>();
  const [nft, setNft] = useState<NFT | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (mintAddress) {
      fetchNFT(mintAddress);
    }
  }, [mintAddress]);

  const fetchNFT = async (address: string) => {
    try {
      const response = await fetch(`/api/nfts/${address}`);
      const result = await response.json();
      
      if (result.success && result.data) {
        setNft(result.data);
      }
    } catch (error) {
      console.error('Failed to fetch NFT:', error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="flex justify-center items-center h-64">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-500"></div>
      </div>
    );
  }

  if (!nft) {
    return (
      <div className="text-center py-12">
        <h2 className="text-2xl font-bold text-white mb-4">NFT Not Found</h2>
        <p className="text-gray-400">Unable to find NFT with address: {mintAddress}</p>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto">
      <div className="mb-8">
        <h1 className="text-4xl font-bold text-white mb-2">{nft.name}</h1>
        <p className="text-gray-400">Symbol: {nft.symbol}</p>
        <p className="text-gray-500 text-sm">Mint: {nft.mint_address}</p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* NFT Visualization */}
        <div className="nft-card">
          <h3 className="text-lg font-semibold text-white mb-4">NFT Visualization</h3>
          <div className="relative">
            <div className="aspect-square bg-gradient-to-br from-purple-600 to-green-600 rounded-lg animate-float">
              <div className="absolute inset-0 flex items-center justify-center">
                <div className="text-center">
                  <div className="text-6xl mb-4">🎨</div>
                  <div className="text-white font-bold text-xl">Living NFT</div>
                  <div className="text-white text-sm mt-2">
                    Power Level: {nft.traits.power_level}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Trait Details */}
        <div className="nft-card">
          <h3 className="text-lg font-semibold text-white mb-4">Trait Details</h3>
          <div className="space-y-4">
            <TraitBar 
              label="Background" 
              value={nft.traits.background} 
              max={255}
              color="from-blue-500 to-purple-500"
            />
            <TraitBar 
              label="Mood" 
              value={nft.traits.mood} 
              max={255}
              color="from-yellow-500 to-orange-500"
            />
            <TraitBar 
              label="Activity" 
              value={nft.traits.activity} 
              max={255}
              color="from-green-500 to-teal-500"
            />
            <TraitBar 
              label="Weather Effect" 
              value={nft.traits.weather_effect} 
              max={255}
              color="from-cyan-500 to-blue-500"
            />
            <TraitBar 
              label="Time of Day" 
              value={nft.traits.time_of_day} 
              max={255}
              color="from-purple-500 to-pink-500"
            />
            <TraitBar 
              label="Special Event" 
              value={nft.traits.special_event} 
              max={255}
              color="from-red-500 to-yellow-500"
            />
            <TraitBar 
              label="Power Level" 
              value={nft.traits.power_level} 
              max={65535}
              color="from-purple-500 to-green-500"
            />
            <TraitBar 
              label="Rarity Score" 
              value={nft.traits.rarity_score} 
              max={65535}
              color="from-pink-500 to-purple-500"
            />
          </div>
        </div>
      </div>

      {/* Additional Information */}
      <div className="mt-8 grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="nft-card">
          <h3 className="text-lg font-semibold text-white mb-4">NFT Information</h3>
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-gray-400">Owner:</span>
              <span className="text-white text-sm">
                {nft.owner_address.slice(0, 4)}...{nft.owner_address.slice(-4)}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-400">Created:</span>
              <span className="text-white text-sm">
                {new Date(nft.created_at).toLocaleDateString()}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-400">Last Updated:</span>
              <span className="text-white text-sm">
                {new Date(nft.updated_at).toLocaleDateString()}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-400">Status:</span>
              <span className={`px-2 py-1 rounded text-xs ${
                nft.is_active ? 'bg-green-500 text-white' : 'bg-red-500 text-white'
              }`}>
                {nft.is_active ? 'Active' : 'Inactive'}
              </span>
            </div>
          </div>
        </div>

        <div className="nft-card">
          <h3 className="text-lg font-semibold text-white mb-4">Trait Analysis</h3>
          <div className="space-y-3">
            <div className="p-3 bg-gray-700 rounded-lg">
              <div className="text-sm text-gray-300 mb-1">Overall Rarity</div>
              <div className="text-xl font-bold text-white">
                {((nft.traits.rarity_score / 65535) * 100).toFixed(1)}%
              </div>
            </div>
            <div className="p-3 bg-gray-700 rounded-lg">
              <div className="text-sm text-gray-300 mb-1">Power Ranking</div>
              <div className="text-xl font-bold text-white">
                #{nft.traits.power_level}
              </div>
            </div>
            <div className="p-3 bg-gray-700 rounded-lg">
              <div className="text-sm text-gray-300 mb-1">Trait Diversity</div>
              <div className="text-xl font-bold text-white">
                {calculateDiversity(nft.traits)}%
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

const calculateDiversity = (traits: NFTTraits): number => {
  const values = [
    traits.background,
    traits.mood,
    traits.activity,
    traits.weather_effect,
    traits.time_of_day,
    traits.special_event
  ];
  
  const uniqueValues = new Set(values).size;
  return Math.round((uniqueValues / 6) * 100);
};
