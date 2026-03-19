import React, { createContext, useContext, useState, useCallback, ReactNode } from 'react';
import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';
import { 
  WalletAdapter, 
  WalletNotConnectedError,
  WalletReadyState 
} from '@solana/wallet-adapter-base';
import { 
  useWallet, 
  WalletContextState 
} from '@solana/wallet-adapter-react';

interface WalletContextType extends WalletContextState {
  connection: Connection;
  connected: boolean;
  publicKey: PublicKey | null;
  connectWallet: () => Promise<void>;
  disconnectWallet: () => Promise<void>;
  sendTransaction: (transaction: any) => Promise<string>;
}

const WalletContext = createContext<WalletContextType | undefined>(undefined);

export const useWalletContext = () => {
  const context = useContext(WalletContext);
  if (!context) {
    throw new Error('useWalletContext must be used within a WalletProvider');
  }
  return context;
};

interface WalletProviderProps {
  children: ReactNode;
}

export const WalletProvider: React.FC<WalletProviderProps> = ({ children }) => {
  const wallet = useWallet();
  const [connection] = useState<Connection>(
    new Connection(clusterApiUrl('devnet'), 'confirmed')
  );

  const connectWallet = useCallback(async () => {
    try {
      if (!wallet.wallet) {
        throw new WalletNotConnectedError('Wallet not found');
      }
      await wallet.connect();
    } catch (error) {
      console.error('Failed to connect wallet:', error);
      throw error;
    }
  }, [wallet]);

  const disconnectWallet = useCallback(async () => {
    try {
      await wallet.disconnect();
    } catch (error) {
      console.error('Failed to disconnect wallet:', error);
      throw error;
    }
  }, [wallet]);

  const sendTransaction = useCallback(async (transaction: any) => {
    if (!wallet.connected || !wallet.signTransaction) {
      throw new WalletNotConnectedError('Wallet not connected');
    }

    try {
      const signedTransaction = await wallet.signTransaction(transaction);
      const signature = await connection.sendRawTransaction(
        signedTransaction.serialize()
      );
      
      await connection.confirmTransaction(signature);
      return signature;
    } catch (error) {
      console.error('Transaction failed:', error);
      throw error;
    }
  }, [wallet, connection]);

  const value: WalletContextType = {
    ...wallet,
    connection,
    connected: wallet.connected,
    publicKey: wallet.publicKey,
    connectWallet,
    disconnectWallet,
    sendTransaction,
  };

  return (
    <WalletContext.Provider value={value}>
      {children}
    </WalletContext.Provider>
  );
};
