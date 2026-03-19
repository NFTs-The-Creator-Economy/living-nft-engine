import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { WalletProvider } from './contexts/WalletContext';
import { Header } from './components/Header';
import { NFTGallery } from './components/NFTGallery';
import { MintNFT } from './components/MintNFT';
import { OracleStatus } from './components/OracleStatus';
import { TraitVisualizer } from './components/TraitVisualizer';
import './index.css';

function App() {
  return (
    <WalletProvider>
      <Router>
        <div className="min-h-screen bg-gray-900">
          <Header />
          <main className="container mx-auto px-4 py-8">
            <Routes>
              <Route path="/" element={<NFTGallery />} />
              <Route path="/mint" element={<MintNFT />} />
              <Route path="/oracle" element={<OracleStatus />} />
              <Route path="/visualizer/:mintAddress" element={<TraitVisualizer />} />
            </Routes>
          </main>
        </div>
      </Router>
    </WalletProvider>
  );
}

export default App;
