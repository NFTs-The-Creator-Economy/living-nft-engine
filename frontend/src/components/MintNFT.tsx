import React, { useState } from 'react';
import { useWalletContext } from '../contexts/WalletContext';
import { MintNFTRequest, NFTTraits } from '../types';

export const MintNFT: React.FC = () => {
  const { connected, publicKey, sendTransaction } = useWalletContext();
  const [loading, setLoading] = useState(false);
  const [formData, setFormData] = useState<MintNFTRequest>({
    name: '',
    symbol: '',
    uri: '',
    owner_address: '',
    initial_traits: {
      background: 0,
      mood: 128,
      activity: 128,
      weather_effect: 0,
      time_of_day: 128,
      special_event: 0,
      power_level: 1000,
      rarity_score: 1000,
    }
  });

  React.useEffect(() => {
    if (publicKey) {
      setFormData(prev => ({
        ...prev,
        owner_address: publicKey.toBase58()
      }));
    }
  }, [publicKey]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!connected) {
      alert('Please connect your wallet first');
      return;
    }

    setLoading(true);
    
    try {
      const response = await fetch('/api/nfts/mint', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(formData),
      });

      const result = await response.json();
      
      if (result.success && result.data) {
        alert(`NFT minted successfully! Transaction: ${result.data.transaction_id}`);
        // Reset form
        setFormData({
          name: '',
          symbol: '',
          uri: '',
          owner_address: publicKey?.toBase58() || '',
          initial_traits: {
            background: 0,
            mood: 128,
            activity: 128,
            weather_effect: 0,
            time_of_day: 128,
            special_event: 0,
            power_level: 1000,
            rarity_score: 1000,
          }
        });
      } else {
        alert(`Minting failed: ${result.error}`);
      }
    } catch (error) {
      console.error('Minting error:', error);
      alert('Minting failed. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: value
    }));
  };

  const handleTraitChange = (trait: keyof NFTTraits, value: number) => {
    setFormData(prev => ({
      ...prev,
      initial_traits: {
        ...prev.initial_traits!,
        [trait]: value
      }
    }));
  };

  if (!connected) {
    return (
      <div className="text-center py-12">
        <h2 className="text-2xl font-bold text-white mb-4">Connect Wallet to Mint</h2>
        <p className="text-gray-400">You need to connect your Solana wallet to mint a Living NFT.</p>
      </div>
    );
  }

  return (
    <div className="max-w-2xl mx-auto">
      <h1 className="text-4xl font-bold text-white mb-8">Mint Living NFT</h1>
      
      <form onSubmit={handleSubmit} className="space-y-6">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              NFT Name
            </label>
            <input
              type="text"
              name="name"
              value={formData.name}
              onChange={handleInputChange}
              className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white focus:outline-none focus:border-purple-500"
              placeholder="My Living NFT"
              required
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Symbol
            </label>
            <input
              type="text"
              name="symbol"
              value={formData.symbol}
              onChange={handleInputChange}
              className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white focus:outline-none focus:border-purple-500"
              placeholder="LIVING"
              maxLength={10}
              required
            />
          </div>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-300 mb-2">
            Metadata URI
          </label>
          <input
            type="url"
            name="uri"
            value={formData.uri}
            onChange={handleInputChange}
            className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white focus:outline-none focus:border-purple-500"
            placeholder="https://example.com/metadata.json"
            required
          />
        </div>

        <div>
          <h3 className="text-lg font-semibold text-white mb-4">Initial Traits</h3>
          <div className="space-y-4">
            {Object.entries(formData.initial_traits!).map(([trait, value]) => (
              <div key={trait}>
                <label className="block text-sm font-medium text-gray-300 mb-2 capitalize">
                  {trait.replace(/_/g, ' ')}
                </label>
                <input
                  type="range"
                  min="0"
                  max={trait === 'power_level' || trait === 'rarity_score' ? 65535 : 255}
                  value={value}
                  onChange={(e) => handleTraitChange(trait as keyof NFTTraits, parseInt(e.target.value))}
                  className="w-full"
                />
                <div className="text-right text-sm text-gray-400">{value}</div>
              </div>
            ))}
          </div>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-300 mb-2">
            Owner Address
          </label>
          <input
            type="text"
            value={formData.owner_address}
            readOnly
            className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-gray-400"
          />
        </div>

        <button
          type="submit"
          disabled={loading}
          className="w-full solana-button disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {loading ? 'Minting...' : 'Mint NFT'}
        </button>
      </form>
    </div>
  );
};
