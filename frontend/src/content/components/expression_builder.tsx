import React, { useState } from 'react';

interface Tag {
  attribute: string;
  operator: string;
  value: string;
  andOr: string; // 'AND', 'OR', or '' for the first condition
}

interface TagBuilderProps {
  onTagsUpdated: (expression: string) => void;
}

interface AttributeOption {
  operators: string[];
  values: string[]; // Specify that values is an array of strings
}

const attributeOptions: Record<string, AttributeOption> = {
  FileType: {
    operators: ['=', '!='],
    values: ['TIFF', 'JPEG', 'PNG', 'GEOJSON', 'KML'],
  },
  Resolution: {
    operators: ['=', '!=', '>', '<', '>=', '<='],
    values: ['5','10','20','50','100'] // Resolution options
  },
  Size: {
    operators: ['>', '<', '>=', '<='],
    values: [] // Assuming numeric input; no predefined values
  }
};

const TagBuilder: React.FC<TagBuilderProps> = ({ onTagsUpdated }) => {
    const [tags, setTags] = useState<Tag[]>([
        {
          attribute: 'FileType',
          operator: attributeOptions.FileType.operators[0],
          value: attributeOptions.FileType.values[0],
          andOr: '', // First condition has no preceding logical operator
        }
      ]);
      
      const addTag = () => {
        const defaultAttribute = 'FileType';
        setTags([...tags, {
          attribute: defaultAttribute,
          operator: attributeOptions[defaultAttribute].operators[0],
          value: attributeOptions[defaultAttribute].values[0],
          andOr: 'AND' // Default new conditions to be combined with "AND"
        }]);
      };
      

  const updateTag = (index: number, updatedPart: Partial<Tag>) => {
    const updatedTags = tags.map((tag, idx) => idx === index ? { ...tag, ...updatedPart } : tag);
    setTags(updatedTags);
  };

  const removeTag = (index: number) => {
    setTags(tags.filter((_, idx) => idx !== index));
  };

  const constructExpression = () => {
    const expression = tags.map((tag, index) => 
      `${index > 0 ? `${tag.andOr} ` : ''}${tag.attribute} ${tag.operator} '${tag.value}'`
    ).join(' ');
    onTagsUpdated(expression);
  };
  

  return (
<div className='mt-2'>
  {tags.map((tag, index) => (
    <div key={index} className="tag-builder-row flex items-center gap-2 mb-4">
    {index > 0 && (
        <select
            className="bg-gray-100 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block p-2"
            value={tag.andOr}
            onChange={e => updateTag(index, { andOr: e.target.value })}
        >
        <option value="AND">AND</option>
        <option value="OR">OR</option>
        </select>
    )}
      <select
        className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5"
        value={tag.attribute}
        onChange={e => {
          const newAttribute = e.target.value;
          const newOperators = attributeOptions[newAttribute].operators;
          const newValue = attributeOptions[newAttribute].values.length > 0 ? attributeOptions[newAttribute].values[0] : '';
          updateTag(index, { attribute: newAttribute, operator: newOperators[0], value: newValue });
        }}
      >
        {Object.keys(attributeOptions).map(attr => (
          <option key={attr} value={attr}>{attr}</option>
        ))}
      </select>

      <select
        className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5"
        value={tag.operator}
        onChange={e => updateTag(index, { operator: e.target.value })}
      >
        {attributeOptions[tag.attribute].operators.map(op => (
          <option key={op} value={op}>{op}</option>
        ))}
      </select>

      {attributeOptions[tag.attribute].values.length > 0 ? (
        <select
          className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5"
          value={tag.value}
          onChange={e => updateTag(index, { value: e.target.value })}
        >
          {attributeOptions[tag.attribute].values.map(val => (
            <option key={val} value={val}>{val}</option>
          ))}
        </select>
      ) : (
        <input
          type="text"
          className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5"
          value={tag.value}
          onChange={e => updateTag(index, { value: e.target.value })}
        />
      )}

      <button
        className="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded"
        onClick={() => removeTag(index)}
      >
        Remove
      </button>
    </div>
  ))}
  <div className="flex gap-2">
    <button
      className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
      onClick={addTag}
    >
      Add Condition
    </button>
    <button
      className="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
      onClick={constructExpression}
    >
      Save Tags
    </button>
  </div>
</div>

  );
};

export default TagBuilder;
