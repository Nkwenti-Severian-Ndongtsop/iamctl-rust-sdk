import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

const config: Config = {
  title: 'iamctl',
  tagline: 'A Rust SDK for building iamctl providers with type safety and performance',
  favicon: 'img/favicon.ico',

  // Future flags, see https://docusaurus.io/docs/api/docusaurus-config#future
  future: {
    v4: true, // Improve compatibility with the upcoming Docusaurus v4
  },

  // Set the production url of your site here
  url: 'https://nkwenti-severian-ndongtsop.github.io',
  baseUrl: '/iamctl-rust-sdk/',
  organizationName: 'Nkwenti-Severian-Ndongtsop',
  projectName: 'iamctl-rust-sdk',

  onBrokenLinks: 'throw',

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/Nkwenti-Severian-Ndongtsop/iamctl-rust-sdk/tree/main/iamctl-docs/',
        },
        blog: {
          showReadingTime: true,
          feedOptions: {
            type: ['rss', 'atom'],
            xslt: true,
          },
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/Nkwenti-Severian-Ndongtsop/iamctl-rust-sdk/tree/main/iamctl-docs/',
          // Useful options to enforce blogging best practices
          onInlineTags: 'warn',
          onInlineAuthors: 'warn',
          onUntruncatedBlogPosts: 'warn',
        },
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    // Replace with your project's social card
    image: 'img/docusaurus-social-card.jpg',
    colorMode: {
      defaultMode: 'light',
      respectPrefersColorScheme: true,
    },
    navbar: {
      title: '',
      logo: {
        alt: 'iamctl Logo',
        src: 'img/logo.png',
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'tutorialSidebar',
          position: 'left',
          label: 'SDK Docs',
        },
        {
          to: '/docs/getting-started',
          label: 'Examples',
          position: 'left',
        },
        {
          to: '/docs/api-reference',
          label: 'API Reference',
          position: 'left',
        },
        {
          to: '/docs/intro',
          label: 'Architecture',
          position: 'left',
        },
        {
          type: 'html',
          position: 'right',
          value:
            '<div class="navbarSearch"><span class="navbarSearch__icon" aria-hidden="true">⌕</span><span class="navbarSearch__text">Search</span><span class="navbarSearch__kbd">Ctrl K</span></div>',
        },
        {
          href: 'https://github.com/Nkwenti-Severian-Ndongtsop/iamctl-rust-sdk',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'SDK Docs',
          items: [
            {
              label: 'Getting Started',
              to: '/docs/getting-started',
            },
            {
              label: 'API Reference',
              to: '/docs/api-reference',
            },
            {
              label: 'Architecture',
              to: '/docs/intro',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              label: 'GitHub',
              href: 'https://github.com/Nkwenti-Severian-Ndongtsop/iamctl-rust-sdk',
            },
            {
              label: 'Discord',
              href: 'https://discord.gg/iamctl',
            },
          ],
        },
        {
          title: 'More',
          items: [
            {
              label: 'Security Policy',
              href: 'https://github.com/Nkwenti-Severian-Ndongtsop/iamctl-rust-sdk/security/policy',
            },
          ],
        },
      ],
      copyright: `Copyright ${new Date().getFullYear()} iamctl. Built with ❤️ for the Rust community.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
