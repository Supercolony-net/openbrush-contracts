const lightCodeTheme = require('prism-react-renderer/themes/vsDark')
const darkCodeTheme = require('prism-react-renderer/themes/vsDark')

/** @type {import('@docusaurus/types').DocusaurusConfig} */
module.exports = {
  title: 'OpenBrush',
  tagline: 'OpenBrush contracts documentation',
  url: 'https://supercolony-net.github.io',
  baseUrl: '/openbrush-contracts/',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  favicon: 'img/favicon.svg',
  organizationName: 'supercolony-net',
  projectName: 'openbrush-contracts',
  themeConfig: {
    navbar: {
      logo: {
        alt: 'OpenBrush',
        src: 'img/logo.svg',
        srcDark: 'img/logo-dark.svg'
      },
      items: [
        {
          type: 'doc',
          docId: 'getting-started',
          position: 'right',
          label: 'Learn'
        },
        {
          href: 'https://github.com/Supercolony-net/openbrush-contracts',
          label: 'GitHub',
          position: 'right'
        }
      ]
    },
    footer: {
      // links: [
      //   {
      //     title: 'Docs',
      //     items: [
      //       {
      //         label: 'Tutorial',
      //         to: '/docs/getting-started'
      //       }
      //     ]
      //   },
      //   {
      //     title: 'Community',
      //     items: [
      //       {
      //         label: 'Stack Overflow',
      //         href: 'https://stackoverflow.com/questions/tagged/docusaurus'
      //       },
      //       {
      //         label: 'Discord',
      //         href: 'https://discordapp.com/invite/docusaurus'
      //       },
      //       {
      //         label: 'Twitter',
      //         href: 'https://twitter.com/docusaurus'
      //       }
      //     ]
      //   }
      // ],
      copyright: `Copyright © ${new Date().getFullYear()} OpenBrush, Supercolony.net.`
    },
    prism: {
      theme: lightCodeTheme,
      darkTheme: darkCodeTheme,
      additionalLanguages: ['toml', 'rust']
    }
  },
  presets: [
    [
      '@docusaurus/preset-classic',
      {
        docs: {
          routeBasePath: '/',
          sidebarPath: require.resolve('./sidebars.js'),
          editUrl: 'https://github.com/Supercolony-net/openbrush-contracts/tree/main/'
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css')
        }
      }
    ]
  ]
}
