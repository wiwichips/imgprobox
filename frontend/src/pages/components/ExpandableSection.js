import React, { useState } from 'react';
//import './ExpandableSection.css';

const ExpandableSection = ({ title, children }) => {
  const [isExpanded, setIsExpanded] = useState(false);

  const toggleExpanded = () => {
    setIsExpanded(!isExpanded);
  };

  return (
    <div className="expandable-section">
      <div className="expandable-section-header" onClick={toggleExpanded}>
        <div className={`triangle ${isExpanded ? 'expanded' : ''}`}></div>
        <h3>{title}</h3>
      </div>
      {isExpanded && <div className="expandable-section-content">{children}</div>}
    </div>
  );
};

export default ExpandableSection;

