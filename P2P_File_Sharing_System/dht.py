class DHT:
    def __init__(self):
        self.dht = {}  # {content_hash: [peer_id1, peer_id2, ...]}

    def announce_content(self, content_hash, peer_id):
        if content_hash not in self.dht:
            self.dht[content_hash] = []
        if peer_id not in self.dht[content_hash]:
            self.dht[content_hash].append(peer_id)
        print(f"Content {content_hash} announced by {peer_id}")

    def get_peers_for_content(self, content_hash):
        return self.dht.get(content_hash, [])

    def get_all_content(self):
        return list(self.dht.keys())
