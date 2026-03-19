import React from 'react';
import { Link } from 'react-router-dom';
import { useWalletContext } from '../contexts/WalletContext';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

export const Header: React.FC = () => {
  const { connected, publicKey } = useWalletContext();

  return (
    <header className="bg-gray-800 border-b border-gray-700">
      <div className="container mx-auto px-4">
        <div className="flex items-center justify-between h-16">
          <div className="flex items-center space-x-8">
            <Link to="/" className="text-2xl font-bold bg-gradient-to-r from-purple-500 to-green-500 bg-clip-text text-transparent">
              Living NFT Engine
            </Link>
            
            <nav className="hidden md:flex space-x-6">
              <Link 
                to="/" 
                className="text-gray-300 hover:text-white transition-colors"
              >
                Gallery
              </Link>
              <Link 
                to="/mint" 
                className="text-gray-300 hover:text-white transition-colors"
              >
                Mint
              </Link>
              <Link 
                to="/oracle" 
                className="text-gray-300 hover:text-white transition-colors"
              >
                Oracle
              </Link>
            </nav>
          </div>

          <div className="flex items-center space-x-4">
            {connected && publicKey && (
              <div className="hidden md:block">
                <span className="text-sm text-gray-400">
                  {publicKey.toBase58().slice(0, 4)}...
                  {publicKey.toBase58().slice(-4)}
                </span>
              </div>
            )}
            
            <WalletMultiButton className="solana-button" />
          </div>
        </div>
      </div>
    </header>
  );
};
