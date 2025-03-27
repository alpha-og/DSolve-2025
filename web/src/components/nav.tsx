import { useState } from "react";
import { Menu, X } from "lucide-react";

export default function Navbar() {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <nav className="fixed top-0 left-0 w-full bg-gray-900 text-white shadow-md z-50">
      <div className="max-w-7xl mx-auto px-6 py-4 flex justify-between items-center">
        {/* Logo */}
        <a href="#" className="text-2xl font-bold">
          Marcker
        </a>

        {/* Desktop Menu */}
        <div className="hidden md:flex space-x-12">
          <a href="#features" className="hover:text-blue-400 transition">
            Features
          </a>
          <a href="#how-it-works" className="hover:text-blue-400 transition">
            How It Works
          </a>
        </div>

        {/* CTA Button */}
        <a
          href="#get-started"
          className="hidden md:inline-block px-5 py-2 bg-blue-500 hover:bg-blue-600 transition rounded-lg font-semibold"
        >
          Get Started
        </a>

        {/* Mobile Menu Button */}
        <button
          className="md:hidden"
          onClick={() => setIsOpen(!isOpen)}
          aria-label="Toggle menu"
        >
          {isOpen ? <X size={28} /> : <Menu size={28} />}
        </button>
      </div>

      {/* Mobile Menu */}
      {isOpen && (
        <div className="md:hidden bg-gray-800 py-4">
          <a href="#features" className="block px-6 py-2 hover:text-blue-400">
            Features
          </a>
          <a
            href="#how-it-works"
            className="block px-6 py-2 hover:text-blue-400"
          >
            How It Works
          </a>
          <a href="#pricing" className="block px-6 py-2 hover:text-blue-400">
            Pricing
          </a>
          <a href="#contact" className="block px-6 py-2 hover:text-blue-400">
            Contact
          </a>
          <a
            href="#get-started"
            className="block px-6 py-2 text-center bg-blue-500 hover:bg-blue-600 transition rounded-lg mt-2"
          >
            Get Started
          </a>
        </div>
      )}
    </nav>
  );
}
