import React from 'react';
import { motion } from 'framer-motion';

const sectionVariants = {
    hidden: { opacity: 0, y: 50 },
    visible: { opacity: 1, y: 0, transition: { duration: 0.8, ease: "easeOut" } },
};

const itemVariants = {
    hidden: { opacity: 0, y: 20 },
    visible: { opacity: 1, y: 0, transition: { duration: 0.6, ease: "easeOut" } },
};

const LandingPageContent: React.FC = () => {
    return (
        <div className="bg-gray-900 text-gray-100">
            {/* Section: Hero */}
            <motion.section
                className="hero min-h-screen flex items-center justify-center text-center px-4 py-16"
                initial="hidden"
                animate="visible"
                variants={sectionVariants}
            >
                <div className="max-w-4xl mx-auto">
                    <motion.h1
                        className="text-5xl md:text-6xl font-bold text-violet-400 leading-tight mb-6"
                        variants={itemVariants}
                    >
                        The On-Chain Sound Economy.
                    </motion.h1>
                    <motion.p
                        className="text-xl md:text-2xl text-gray-200 mb-10"
                        variants={itemVariants}
                    >
                        Contribute audio assets, provide hardware resources, and capture protocol value. Join a decentralized network where your contributions are rewarded directly in ECHO.
                    </motion.p>
                    <motion.div
                        className="flex flex-col sm:flex-row justify-center items-center gap-4"
                        variants={itemVariants}
                    >
                        <a href="#" className="bg-violet-600 hover:bg-violet-700 text-white font-bold py-3 px-8 rounded-lg transition duration-300 ease-in-out text-lg">Download Node App (macOS)</a>
                        <a href="#" className="text-teal-400 border border-teal-400 hover:bg-teal-400 hover:text-gray-900 font-bold py-3 px-8 rounded-lg transition duration-300 ease-in-out text-lg">Join Mobile Waitlist</a>
                    </motion.div>
                </div>
            </motion.section>

            {/* Section: The EchoChain Flywheel */}
            <motion.section
                className="flywheel py-20 px-4 bg-gray-800"
                initial="hidden"
                whileInView="visible"
                viewport={{ once: true, amount: 0.3 }}
                variants={sectionVariants}
            >
                <div className="max-w-6xl mx-auto text-center">
                    <motion.h2
                        className="text-4xl font-bold text-violet-400 mb-12"
                        variants={itemVariants}
                    >
                        The EchoChain Flywheel
                    </motion.h2>
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-10">
                        <motion.div className="p-8 rounded-lg shadow-lg bg-gray-900 border border-teal-400" variants={itemVariants}>
                            <div className="text-teal-400 text-5xl mb-6">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth="1.5" stroke="currentColor" className="w-12 h-12 mx-auto">
                                    <path strokeLinecap="round" strokeLinejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                                </svg>
                            </div>
                            <h3 className="text-2xl font-semibold text-gray-100 mb-4">Contribute Assets</h3>
                            <p className="text-gray-200">Onboard your original audio assets via our macOS app. Your creations become part of a permanent, peer-to-peer library.</p>
                        </motion.div>
                        <motion.div className="p-8 rounded-lg shadow-lg bg-gray-900 border border-teal-400" variants={itemVariants}>
                            <div className="text-teal-400 text-5xl mb-6">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth="1.5" stroke="currentColor" className="w-12 h-12 mx-auto">
                                    <path strokeLinecap="round" strokeLinejoin="round" d="M2.25 18.75a60.07 60.07 0 0 1 15.795 2.104c.207.055.42.055.627 0a60.07 60.07 0 0 0 15.795-2.104M2.25 18.75V5.25A2.25 2.25 0 0 1 4.5 3h15A2.25 2.25 0 0 1 21.75 5.25v13.5m-13.5-3.375c0-.621.504-1.125 1.125-1.125h.375c.621 0 1.125.504 1.125 1.125v.375c0 .621-.504 1.125-1.125 1.125h-.375a1.125 1.125 0 0 1-1.125-1.125v-.375Zm6.75 0c0-.621.504-1.125 1.125-1.125h.375c.621 0 1.125.504 1.125 1.125v.375c0 .621-.504 1.125-1.125 1.125h-.375a1.125 1.125 0 0 1-1.125-1.125v-.375Z" />
                                </svg>
                            </div>
                            <h3 className="text-2xl font-semibold text-gray-100 mb-4">Earn Protocol Rewards</h3>
                            <p className="text-gray-200">Receive ECHO tokens through automated, on-chain distributions for your contributions. All value is generated and distributed by the protocol.</p>
                        </motion.div>
                        <motion.div className="p-8 rounded-lg shadow-lg bg-gray-900 border border-teal-400" variants={itemVariants}>
                            <div className="text-teal-400 text-5xl mb-6">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth="1.5" stroke="currentColor" className="w-12 h-12 mx-auto">
                                    <path strokeLinecap="round" strokeLinejoin="round" d="M8.25 3v1.5M4.5 8.25H3m7.5 0H12m9-3.75v1.5m0 4.5h-1.5M18 11.25h-1.5m-4.5 0H12m-7.5 0H3m9 6.75V12m0 3.75h.008v.008H12v-.008Zm0 3.75h.008v.008H12v-.008Zm0 3.75h.008v.008H12v-.008ZM3 12h.008v.008H3V12Zm0 3.75h.008v.008H3v-.008Zm0 3.75h.008v.008H3v-.008Zm3.75-9h.008v.008H6.75V12Zm3.75 0h.008v.008H10.5V12Zm3.75 0h.008v.008H14.25V12Zm3.75 0h.008v.008H18V12Zm3.75 0h.008v.008H21.75V12Zm-3.75 3.75h.008v.008H18v-.008Zm-3.75 0h.008v.008H14.25v-.008Zm-3.75 0h.008v.008H10.5v-.008Zm-3.75 0h.008v.008H6.75v-.008Z" />
                                </svg>
                            </div>
                            <h3 className="text-2xl font-semibold text-gray-100 mb-4">Power the Network</h3>
                            <p className="text-gray-200">Retrieve samples instantly from the decentralized storage layer. Soon, you can provide mobile resources to strengthen the network and increase your rewards.</p>
                        </motion.div>
                    </div>
                </div>
            </motion.section>

            {/* Section: Application Showcase */}
            <section className="app-showcase py-20 px-4">
                <div className="max-w-6xl mx-auto">
                    {/* Part 1: Desktop Node */}
                    <motion.div
                        className="grid grid-cols-1 md:grid-cols-2 gap-12 items-center mb-20"
                        initial="hidden"
                        whileInView="visible"
                        viewport={{ once: true, amount: 0.3 }}
                        variants={sectionVariants}
                    >
                        <motion.div className="order-2 md:order-1" variants={itemVariants}>
                            <h2 className="text-4xl font-bold text-violet-400 mb-6">Your Gateway to the Protocol.</h2>
                            <ul className="list-disc list-inside text-gray-200 text-lg space-y-3 mb-8">
                                <li>Seamlessly onboard new audio assets.</li>
                                <li>Manage your on-chain identity and ECHO wallet.</li>
                                <li>Access the peer-to-peer sample library.</li>
                                <li>Functions as a full node, supporting the network.</li>
                            </ul>
                            <a href="#" className="bg-violet-600 hover:bg-violet-700 text-white font-bold py-3 px-8 rounded-lg transition duration-300 ease-in-out text-lg">Download for macOS</a>
                        </motion.div>
                        <motion.div className="order-1 md:order-2" variants={itemVariants}>
                            <img src="https://placehold.co/1024x768/111827/F9FAFB?text=EchoChain+macOS+App" alt="EchoChain macOS App" className="rounded-lg shadow-xl w-full" />
                        </motion.div>
                    </motion.div>

                    {/* Part 2: Mobile Lite-Client */}
                    <motion.div
                        className="grid grid-cols-1 md:grid-cols-2 gap-12 items-center"
                        initial="hidden"
                        whileInView="visible"
                        viewport={{ once: true, amount: 0.3 }}
                        variants={sectionVariants}
                    >
                        <motion.div className="order-2 md:order-2" variants={itemVariants}>
                            <h2 className="text-4xl font-bold text-violet-400 mb-6">Earn Passively from your Pocket.</h2>
                            <p className="text-gray-200 text-lg mb-8">Our upcoming iOS and Android clients allow you to earn protocol rewards by securely delegating a portion of your phone's storage and bandwidth. The client runs opportunistically, turning your idle device into a productive network resource.</p>
                            <form className="flex flex-col sm:flex-row gap-4">
                                <input type="email" placeholder="Enter your email" className="flex-grow p-3 rounded-lg bg-gray-800 border border-teal-400 text-gray-100 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-teal-400" />
                                <button type="submit" className="bg-violet-600 hover:bg-violet-700 text-white font-bold py-3 px-8 rounded-lg transition duration-300 ease-in-out text-lg">Join the Waitlist</button>
                            </form>
                        </motion.div>
                        <motion.div className="order-1 md:order-1" variants={itemVariants}>
                            <img src="https://placehold.co/768x1024/111827/F9FAFB?text=EchoChain+Mobile+App" alt="EchoChain Mobile App" className="rounded-lg shadow-xl w-full" />
                        </motion.div>
                    </motion.div>
                </div>
            </section>

            {/* Section: The EchoChain Ecosystem */}
            <motion.section
                className="ecosystem py-20 px-4 bg-gray-800"
                initial="hidden"
                whileInView="visible"
                viewport={{ once: true, amount: 0.3 }}
                variants={sectionVariants}
            >
                <div className="max-w-6xl mx-auto text-center">
                    <motion.h2
                        className="text-4xl font-bold text-violet-400 mb-12"
                        variants={itemVariants}
                    >
                        The EchoChain Ecosystem
                    </motion.h2>
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-10">
                        <motion.div className="p-8 rounded-lg shadow-lg bg-gray-900 border border-teal-400" variants={itemVariants}>
                            <h3 className="text-2xl font-semibold text-gray-100 mb-4">Asset Providers (Creators)</h3>
                            <p className="text-gray-200">The heart of the library. Onboard original sounds and receive on-chain incentives.</p>
                        </motion.div>
                        <motion.div className="p-8 rounded-lg shadow-lg bg-gray-900 border border-teal-400" variants={itemVariants}>
                            <h3 className="text-2xl font-semibold text-gray-100 mb-4">Users</h3>
                            <p className="text-gray-200">Discover unique sounds for your projects, retrieved from a decentralized storage layer.</p>
                        </motion.div>
                        <motion.div className="p-8 rounded-lg shadow-lg bg-gray-900 border border-teal-400" variants={itemVariants}>
                            <h3 className="text-2xl font-semibold text-gray-100 mb-4">Infrastructure Providers (You)</h3>
                            <p className="text-gray-200">Provide hardware resources (desktop or mobile) and capture protocol value.</p>
                        </motion.div>
                    </div>
                </div>
            </motion.section>

            {/* Section: Final Call-to-Action & Footer */}
            <motion.section
                className="final-cta py-20 px-4 text-center"
                initial="hidden"
                whileInView="visible"
                viewport={{ once: true, amount: 0.3 }}
                variants={sectionVariants}
            >
                <div className="max-w-4xl mx-auto">
                    <motion.h2
                        className="text-4xl font-bold text-violet-400 mb-10"
                        variants={itemVariants}
                    >
                        Ready to participate in the new sound economy?
                    </motion.h2>
                    <motion.div
                        className="flex flex-col sm:flex-row justify-center items-center gap-4 mb-16"
                        variants={itemVariants}
                    >
                        <a href="#" className="bg-violet-600 hover:bg-violet-700 text-white font-bold py-3 px-8 rounded-lg transition duration-300 ease-in-out text-lg">Download Node App (macOS)</a>
                        <a href="#" className="text-teal-400 border border-teal-400 hover:bg-teal-400 hover:text-gray-900 font-bold py-3 px-8 rounded-lg transition duration-300 ease-in-out text-lg">Join Mobile Waitlist</a>
                    </motion.div>

                    <footer className="text-gray-400 text-sm">
                        <div className="flex justify-center space-x-6">
                            <a href="#" className="hover:text-gray-100 transition duration-300 ease-in-out">Litepaper</a>
                            <a href="#" className="hover:text-gray-100 transition duration-300 ease-in-out">Community</a>
                            <a href="#" className="hover:text-gray-100 transition duration-300 ease-in-out">Twitter</a>
                        </div>
                    </footer>
                </div>
            </motion.section>
        </div>
    );
};

export default LandingPageContent;
