import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    {
      type: 'category',
      label: 'Getting Started',
      items: ['intro', 'getting-started'],
    },
    {
      type: 'category',
      label: 'Concepts',
      items: [
        'concepts/engine-provider',
        'concepts/resources-changes',
        'concepts/state',
        'concepts/validation',
      ],
    },
    {
      type: 'category',
      label: 'Provider Development',
      items: ['provider-development/implementing-a-provider'],
    },
    {
      type: 'category',
      label: 'Reference',
      items: ['api-reference', 'reference/json-rpc'],
    },
  ],
};

export default sidebars;
