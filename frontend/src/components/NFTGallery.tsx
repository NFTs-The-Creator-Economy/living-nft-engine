import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { NFT } from '../types';
import { TraitBar } from './TraitBar';

export const NFTGallery: React.FC = () => {
  const [nfts, setNfts] = useState<NFT[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchNFTs();
  }, []);

  const fetchNFTs = async () => {
    try {
      const response = await fetch('/api/nfts');
      const result = await response.json();
      
      if (result.success && result.data) {
        setNfts(result.data.items);
      }
    } catch (error) {
      console.error('Failed to fetch NFTs:', error);
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

  return (
    <div>
      <div className="flex justify-between items-center mb-8">
        <h1 className="text-4xl font-bold text-white">Living NFT Gallery</h1>
        <Link 
          to="/mint" 
          className="solana-button"
        >
          Mint New NFT
        </Link>
      </div>

      {nfts.length === 0 ? (
        <div className="text-center py-12">
          <div className="text-gray-400 mb-4">No NFTs found</div>
          <Link 
            to="/mint" 
            className="solana-button"
          >
            Create Your First Living NFT
          </Link>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {nfts.map((nft) => (
            <NFTCard key={nft.id} nft={nft} />
          ))}
        </div>
      )}
    </div>
  );
};

interface NFTCardProps {
  nft: NFT;
}

const NFTCard: React.FC<NFTCardProps> = ({ nft }) => {
  return (
    <div className="nft-card">
      <div className="relative mb-4">
        <div className="aspect-square bg-gradient-to-br from-purple-600 to-green-600 rounded-lg"></div>
        <div className="absolute top-2 right-2 bg-black bg-opacity-50 px-2 py-1 rounded text-xs text-white">
          Power: {nft.traits.power_level}
        </div>
      </div>
      
      <div className="mb-4">
        <h3 className="text-xl font-bold text-white mb-1">{nft.name}</h3>
        <p className="text-gray-400 text-sm">{nft.symbol}</p>
      </div>

      <div className="space-y-3">
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
          label="Rarity" 
          value={nft.traits.rarity_score} 
          max={65535}
          color="from-purple-500 to-pink-500"
        />
      </div>

      <div className="mt-4 pt-4 border-t border-gray-700">
        <Link 
          to={`/visualizer/${nft.mint_address}`}
          className="block w-full text-center bg-gray-700 hover:bg-gray-600 text-white py-2 rounded transition-colors"
        >
          View Details
        </Link>
      </div>

      <div className="mt-2 text-xs text-gray-500">
        Last updated: {new Date(nft.updated_at).toLocaleDateString()}
      </div>
    </div>
  );
};
