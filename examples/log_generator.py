#!/usr/bin/env python3
"""
Script de exemplo que gera logs para testar o Log Collector.
Este script imprime logs para stdout e stderr em intervalos regulares.
"""

import sys
import time
from datetime import datetime


def main():
    print("Log Generator iniciado!", flush=True)
    print("Gerando logs a cada 2 segundos...", flush=True)

    counter = 0

    try:
        while True:
            counter += 1
            timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

            # A cada 3 logs, envia um para stderr
            if counter % 3 == 0:
                print(
                    f"[ERROR] {timestamp} - Erro simulado #{counter}",
                    file=sys.stderr,
                    flush=True,
                )
            else:
                print(f"[INFO] {timestamp} - Log de informação #{counter}", flush=True)

            # Informações adicionais a cada 5 logs
            if counter % 5 == 0:
                print(
                    f"[DEBUG] {timestamp} - Debug: Total de logs gerados: {counter}",
                    flush=True,
                )

            time.sleep(2)

    except KeyboardInterrupt:
        print("\nLog Generator finalizado!", flush=True)
        sys.exit(0)


if __name__ == "__main__":
    main()
