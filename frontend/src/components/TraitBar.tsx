import React from 'react';

interface TraitBarProps {
  label: string;
  value: number;
  max: number;
  color: string;
}

export const TraitBar: React.FC<TraitBarProps> = ({ label, value, max, color }) => {
  const percentage = (value / max) * 100;

  return (
    <div>
      <div className="flex justify-between text-sm mb-1">
        <span className="text-gray-300">{label}</span>
        <span className="text-gray-500">{value}</span>
      </div>
      <div className="trait-bar">
        <div 
          className={`trait-fill bg-gradient-to-r ${color}`}
          style={{ width: `${percentage}%` }}
        ></div>
      </div>
    </div>
  );
};
