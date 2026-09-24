/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        brand: {
          primary: '#6366F1', // Indigo
          hover: '#4F46E5',
          soft: '#EEF2FF',
          darkSoft: '#1E1B4B',
        },
        surface: {
          light: {
            bg: '#F7F8FA',
            sidebar: '#FAFAFB',
            card: '#FFFFFF',
            elevated: '#FFFFFF',
            border: '#E5E7EB',
            hover: '#F3F4F6',
          },
          dark: {
            bg: '#0D0F12',
            sidebar: '#111318',
            card: '#15181E',
            elevated: '#191D24',
            border: '#262B33',
            hover: '#20242C',
          }
        },
        text: {
          light: {
            primary: '#18181B',
            secondary: '#52525B',
            muted: '#A1A1AA',
          },
          dark: {
            primary: '#F4F4F5',
            secondary: '#A1A1AA',
            muted: '#71717A',
          }
        },
        semantic: {
          success: '#22C55E',
          warning: '#F59E0B',
          error: '#EF4444',
          info: '#3B82F6',
        }
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'Fira Code', 'monospace'],
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'fade-in': 'fadeIn 0.3s ease-out',
        'slide-up': 'slideUp 0.3s ease-out',
        'spin-slow': 'spin 3s linear infinite',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(8px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
      },
      backdropBlur: {
        xs: '2px',
      },
    },
  },
  plugins: [],
}
