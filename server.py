from flask import Flask, jsonify, request
from Board import GameBoard
from dotenv import load_dotenv
import os

app = Flask(__name__)
load_dotenv()

PORT = os.getenv('PORT')
DEBUG = os.getenv('DEBUG')

@app.route('/gen', methods=['POST'])
def gen():
    data = request.get_json()
    if not data or 'width' not in data or 'height' not in data or 'bombs' not in data:
        return jsonify({"error": "Missing required parameters: width, height, and bombs must all be provided."}), 400

    width = data['width']
    height = data['height']
    bombs = data['bombs']

    if bombs >= width * height:
        return jsonify({"error": "Invalid parameters: number of bombs must be less than total cells (width * height)."}), 400

    game_board, initial_click, attempts, solved = GameBoard.generate_noguess_board(width, height, bombs)

    return jsonify({
        'board': game_board._board.get_board(),
        'initial_click': initial_click,
        'attempts': attempts,
        'solved': solved
    })

if __name__ == '__main__':
    app.run(debug=DEBUG, host='0.0.0.0', port=PORT)
