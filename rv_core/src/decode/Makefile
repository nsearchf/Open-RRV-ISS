all:
	python3 gen_instr_entry.py ../execute/rv_i.rs > tmp.txt
	python3 gen_instr_entry.py ../execute/rv_system.rs >> tmp.txt
	python3 gen_instr_entry.py ../execute/rv_zicsr.rs >> tmp.txt
	python3 gen_instr_entry.py ../execute/rv32_i.rs >> tmp.txt

clean:
	rm -f tmp.txt