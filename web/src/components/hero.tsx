export default function HeroSection() {
  return (
    <section className="relative flex flex-col items-center justify-center h-screen text-center px-6 bg-gradient-to-b from-gray-900 to-black text-white">
      <div className="max-w-4xl">
        <h1 className="text-5xl font-bold leading-tight">
          Decentralized Trust for Every Transaction
        </h1>
        <p className="my-4 text-lg text-gray-300">
          Marcker ensures sellers’ claims are authentic through
          blockchain-powered verification, making trust truly transparent.
        </p>
        <div className="flex justify-center gap-4">
          <button className="px-6 py-3 bg-blue-500 hover:bg-blue-600 transition rounded-lg text-white font-semibold">
            Get Started
          </button>
          <button className="px-6 py-3 bg-gray-800 hover:bg-gray-700 transition rounded-lg text-white font-semibold">
            Learn More
          </button>
        </div>
      </div>
      <div className="absolute top-0 left-0 w-full h-full bg-grid-white/[0.05] pointer-events-none"></div>
    </section>
  );
}
