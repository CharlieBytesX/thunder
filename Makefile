DB_URL = sqlite://bakery.db
OUTPUT_DIR = entity/src/entities

# Phony target to ensure the command always runs, regardless of file names
.PHONY: entities migrate

# Target to generate the entities
entities:
	@echo "Generating entities from $(DB_URL)..."
	sea-orm-cli generate entity -u $(DB_URL) -o $(OUTPUT_DIR)
	@echo "Done! Entities generated in $(OUTPUT_DIR)."



migrate:
	sea-orm-cli migrate up -u $(DB_URL)
	$(MAKE) entities


