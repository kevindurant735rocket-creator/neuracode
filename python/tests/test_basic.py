"""Basic tests for NeuraCode AI package."""

import unittest

from neuracode import __version__
from neuracode.models.classifier import TaskClassifier, TaskType
from neuracode.models.recommender import CodeRecommender
from neuracode.models.summarizer import CodeSummarizer


class TestVersion(unittest.TestCase):
    def test_version(self):
        self.assertEqual(__version__, "0.1.0")


class TestTaskClassifier(unittest.TestCase):
    def setUp(self):
        self.classifier = TaskClassifier()

    def test_classify_bug_fix(self):
        task_type, confidence = self.classifier.classify("fix login bug")
        self.assertEqual(task_type, TaskType.BUG_FIX)
        self.assertGreater(confidence, 0)

    def test_classify_refactor(self):
        task_type, _ = self.classifier.classify("refactor user module")
        self.assertEqual(task_type, TaskType.REFACTOR)

    def test_classify_feature(self):
        task_type, _ = self.classifier.classify("add new feature")
        self.assertEqual(task_type, TaskType.NEW_FEATURE)

    def test_classify_unknown(self):
        task_type, _ = self.classifier.classify("xyzzy")
        self.assertEqual(task_type, TaskType.GENERAL)


class TestCodeSummarizer(unittest.TestCase):
    def setUp(self):
        self.summarizer = CodeSummarizer()

    def test_summarize_function(self):
        summary = self.summarizer.summarize_function(
            "authenticate",
            "def authenticate(user, password):\n    return check(user, password)\n",
        )
        self.assertIn("authenticate", summary.title)
        self.assertTrue(summary.key_points)


class TestCodeRecommender(unittest.TestCase):
    def setUp(self):
        self.recommender = CodeRecommender()

    def test_detect_eval(self):
        recs = self.recommender.analyze("result = eval(user_input)")
        self.assertTrue(any("eval" in r.title for r in recs))

    def test_clean_code(self):
        recs = self.recommender.analyze("value = 1\nprint(value)\n")
        # print triggers a best-practice recommendation
        self.assertIsInstance(recs, list)


if __name__ == "__main__":
    unittest.main()
