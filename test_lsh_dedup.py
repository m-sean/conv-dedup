import csv
import itertools
import unittest

from lsh_dedup import MinHashLSH, DeduplicationTable, Record

TEST_FILE = "test_resources/texts.csv"


def _read_csv_records():
    with open(TEST_FILE, "r") as src:
        return list(Record(*row) for row in csv.reader(src))


class LSHDedupTestCase(unittest.TestCase):
    THRESHOLD = 0.5
    STRICT_THRESHOLD = 1.0
    RECORDS = _read_csv_records()
    LSH = MinHashLSH(RECORDS, 8, 1)
    MINHASHES = LSH.get_minhash_map()
    DEDUP_TABLE = DeduplicationTable(LSH, threshold=THRESHOLD)
    GROUPED_IDS = DEDUP_TABLE.grouped_ids()

    def test_hash_count(self):
        self.assertEqual(len(self.MINHASHES), len(self.RECORDS))

    def test_lsh_query(self):
        for rec_id, minhash in self.MINHASHES.items():
            self.assertIn(rec_id, self.LSH.query(minhash))

    def test_lsh_query_threshold(self):
        for rec_id, minhash in self.MINHASHES.items():
            result = self.LSH.query(minhash, self.THRESHOLD)
            self.assertIn(rec_id, result)
            for res_rec_id in result:
                res_hash = self.MINHASHES[res_rec_id]
                self.assertGreaterEqual(
                    minhash.jaccard_similarity(res_hash), self.THRESHOLD
                )

    def test_lsh_query_threshold_strict(self):
        for rec_id, minhash in self.MINHASHES.items():
            result = self.LSH.query(minhash, self.STRICT_THRESHOLD)
            self.assertIn(rec_id, result)
            for res_rec_id in result:
                res_hash = self.MINHASHES[res_rec_id]
                self.assertEqual(
                    minhash.jaccard_similarity(res_hash), self.STRICT_THRESHOLD
                )

    def test_dedup_table_groups_ct(self):
        self.assertGreater(len(self.GROUPED_IDS), 2)

    def test_dedup_table_groups_lens(self):
        for group in self.GROUPED_IDS:
            self.assertGreater(len(group), 0)

    def test_dedup_table_union(self):
        groups_union = set()
        for grp in self.GROUPED_IDS:
            groups_union |= set(grp)
        self.assertEqual(set([r.uuid for r in self.RECORDS]), groups_union)

    def test_dedup_table_intersection(self):
        for ref, cmp in itertools.combinations_with_replacement(
            self.GROUPED_IDS, 2
        ):
            if ref != cmp:
                self.assertEqual(set(ref).intersection(cmp), set())
