import React from "react";

interface FilterProps {
  onFilterChange: (filter: string) => void;
}

const Filter: React.FC<FilterProps> = ({ onFilterChange }) => {
  return (
    <input
      type="text"
      placeholder="Filter items..."
      onChange={(e) => onFilterChange(e.target.value)}
    />
  );
};

export default Filter;
